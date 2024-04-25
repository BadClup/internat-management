import 'package:flutter/material.dart';
import 'package:internat_management/shared/navbar.dart';

class ChatScreen extends StatelessWidget {
  const ChatScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
        appBar: SharedAppBar(),
        body: Text("Chat screen")
    );
  }
}
