# OpenDesk — Video & Audio Streaming Subsystems

## 1. Hardware Video Encoding Architecture

Real-time remote desktop requires encoding display frames within a strict 2–5 millisecond encoding budget to preserve sub-30ms glass-to-glass latency.

```mermaid
graph TD
    subgraph ColorConversion["Zero-Copy GPU Surface Processing"]
        RawTexture["Captured GPU Texture (DirectX / Metal / DmaBuf)\nBGRA8 / RGBA8"]
        ComputeShader["DirectX / Metal / OpenGL Compute Shader\nColor Space Conversion"]
        NV12Surface["Planar NV12 / YUV420p GPU Surface"]
    end

    subgraph Encoders["Hardware & Software Video Encoders"]
        NVENC["NVIDIA NVENC (NVEncodeAPI)"]
        AMF["AMD Advanced Media Framework (AMF)"]
        QSV["Intel QuickSync / OneVPL"]
        VToolbox["Apple VideoToolbox (VTCompressionSession)"]
        VAAPI["Linux VAAPI (libva)"]
        OpenH264["Software Fallback: OpenH264 / SVT-AV1"]
    end

    RawTexture --> ComputeShader
    ComputeShader --> NV12Surface
    NV12Surface --> NVENC
    NV12Surface --> AMF
    NV12Surface --> QSV
    NV12Surface --> VToolbox
    NV12Surface --> VAAPI
    NV12Surface --> OpenH264
```

---

## 2. Hardware Encoder Parameters & Optimization

### NVIDIA NVENC Configuration
- **API**: NVIDIA Video Codec SDK (NVEncodeAPI).
- **Preset**: `NV_ENC_PRESET_P1` or `NV_ENC_PRESET_P2` (Lowest latency, single-pass).
- **Tuning Info**: `NV_ENC_TUNING_INFO_ULTRA_LOW_LATENCY`.
- **Rate Control**: `NV_ENC_PARAMS_RC_CBR` (Constant Bitrate) with dynamic bitrate adjustment.
- **GOP Structure**:
  - `gopLength = NVENC_INFINITE_GOPLENGTH`.
  - Continuous P-frames; IDR keyframes emitted strictly on session start or explicit RTCP Picture Loss Indication (PLI).
  - B-frames disabled (`frameIntervalP = 1`) to eliminate frame reordering latency.

### Apple VideoToolbox Configuration
- **API**: `VTCompressionSessionCreate`.
- **Properties**:
  ```c
  VTSessionSetProperty(session, kVTCompressionPropertyKey_RealTime, kCFBooleanTrue);
  VTSessionSetProperty(session, kVTCompressionPropertyKey_ProfileLevel, kVTProfileLevel_H264_High_AutoLevel);
  VTSessionSetProperty(session, kVTCompressionPropertyKey_AverageBitRate, (__bridge CFNumberRef)@(targetBitrateBps));
  VTSessionSetProperty(session, kVTCompressionPropertyKey_MaxKeyFrameInterval, (__bridge CFNumberRef)@(3600)); // Keyframes on demand
  VTSessionSetProperty(session, kVTCompressionPropertyKey_AllowFrameReordering, kCFBooleanFalse);
  ```

---

## 3. Audio Streaming Architecture (Opus Codec)

```mermaid
sequenceDiagram
    autonumber
    participant AudioDev as OS Audio Hardware
    participant Capture as Audio Loopback Capturer
    participant Resampler as 48 kHz Linear Resampler
    participant Opus as libopus Encoder
    participant WebRTC as WebRTC Audio Track
    participant ClientDec as Client Opus Decoder
    participant OutputDev as Client Speakers / Headphones

    AudioDev->>Capture: WASAPI Loopback / CoreAudio Tap / PipeWire Sink
    Capture->>Resampler: 44.1 kHz / 48 kHz PCM Chunks
    Resampler->>Opus: Normalized 48 kHz Stereo Float32 (20ms / 960 samples)
    Note over Opus: Encode with Opus VBR<br/>Bitrate: 48 kbps, Complexity: 5, FEC enabled
    Opus->>WebRTC: Encoded Opus Packet (90 - 150 bytes)
    WebRTC->>ClientDec: Transport via SRTP
    ClientDec->>OutputDev: Decoded PCM playback synchronized with video
```

### Audio Specifications:
- **Sample Rate**: 48,000 Hz stereo.
- **Frame Duration**: 20 milliseconds (960 samples per channel).
- **Codec Profile**: `OPUS_APPLICATION_RESTRICTED_LOWDELAY` (minimizes internal algorithmic delay to < 5ms).
- **Forward Error Correction (FEC)**: In-band FEC enabled to reconstruct lost audio packets seamlessly over lossy wireless networks.
