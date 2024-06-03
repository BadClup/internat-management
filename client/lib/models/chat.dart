import 'dart:convert';

import 'package:equatable/equatable.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:http/http.dart' as http;

class Message extends Equatable {
  final String content;
  final int id;
  final int recipientId;
  final int senderId;
  final String createdAt;

  const Message(
      {required this.recipientId,
      required this.senderId,
      required this.createdAt,
      required this.id,
      required this.content});

  factory Message.fromJson(Map<String, dynamic> json) {
    return switch (json) {
      {
        'content': String content,
        'id': int id,
        'recipient_id': int recipientId,
        'sender_id': int senderId,
        'created_at': String createdAt,
      } =>
        Message(
            content: content,
            id: id,
            createdAt: createdAt,
            recipientId: recipientId,
            senderId: senderId),
      _ => throw const FormatException("Failed to load Message"),
    };
  }

  @override
  List get props => [content, id, recipientId, senderId, createdAt];
}

Future<List<Message>> getUserMessages(int userId, String bearerToken) async {
  final apiPrefix = dotenv.env["API_URL"];
  final url = Uri.parse('$apiPrefix/chat/$userId');

  final response =
      await http.get(url, headers: {'Authorization': 'Bearer $bearerToken'});

  if (response.statusCode >= 400) {
    throw Exception("Failed to get user messages. Details: ${response.body}");
  }

  List<dynamic> jsonList = jsonDecode(response.body);
  List<Message> messages = jsonList.map((json) => Message.fromJson(json)).toList();

  return messages;
}

Future<void> sendMessage(int residentId, String bearerToken, String content) async {
  final apiPrefix = dotenv.env["API_URL"];
  final url = Uri.parse('$apiPrefix/chat');
  final body = jsonEncode({"content": content, "resident_id": residentId});

  final response =
  await http.post(url, body: body ,headers: { 'Content-Type': 'application/json','Authorization': 'Bearer $bearerToken'});

  if(response.statusCode != 200) {
    print(response.statusCode);
    throw Exception("Failed to send Message");
  }
}