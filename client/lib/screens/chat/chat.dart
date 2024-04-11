import 'package:flutter/material.dart';

class ChatScreen extends StatelessWidget {
  const ChatScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const SizedBox(
          width: double.infinity,
          child: Text("Internat management", textAlign: TextAlign.right),
        ),
      ),
      body: const Text("Chat screen"),
    );
  }
}
