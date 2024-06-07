import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:internat_management/shared/navbar.dart';

class Contact {
  
  final String firstName;
  final String lastName;
  final int id;
  
  const Contact(this.firstName, this.lastName, this.id);
  
}

class ChatGroups extends StatelessWidget {
  const ChatGroups({super.key});

  @override
  Widget build(BuildContext context) {

    final contacts = [
      const Contact("Bartlomiej", "Strama", 1)
    ];

    final contactsList = contacts.map((contact) {
      return ListTile(
        leading: const Icon(Icons.person),
        title: Text("${contact.firstName} ${contact.lastName}"),
        onTap: () => context.go("/supervisor/chat/${contact.id}")
      );
    }).toList();

    return Scaffold(
      appBar: const SharedAppBar(),
      body: Expanded(child: ListView(
        children: contactsList,
      )),
    );
  }
}
