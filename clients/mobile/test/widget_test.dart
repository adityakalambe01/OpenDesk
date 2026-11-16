import 'package:material_ui/material_ui.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:opendesk_mobile/main.dart';

void main() {
  testWidgets('renders the OpenDesk mobile scaffold', (tester) async {
    await tester.pumpWidget(const OpenDeskApp());

    expect(find.text('OpenDesk mobile scaffold'), findsOneWidget);
    expect(find.byType(MaterialApp), findsOneWidget);
  });
}
