import 'package:flutter/material.dart';

void main() => runApp(const OpenDeskApp());

class OpenDeskApp extends StatelessWidget {
  const OpenDeskApp({super.key});

  @override
  Widget build(BuildContext context) => MaterialApp(
        title: 'OpenDesk',
        theme: ThemeData(colorSchemeSeed: Colors.indigo, useMaterial3: true),
        home: const Scaffold(body: Center(child: Text('OpenDesk mobile scaffold'))),
      );
}
