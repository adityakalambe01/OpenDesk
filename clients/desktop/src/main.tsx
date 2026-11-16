import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';

function App() {
  return <main><h1>OpenDesk</h1><p>Desktop client scaffold</p></main>;
}

createRoot(document.getElementById('root')!).render(<StrictMode><App /></StrictMode>);
