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

class Conversation extends Equatable {
  final int recipientId;
  final int? senderId;
  final String? recentMessageDate;
  final String? recentMessage;

  const Conversation(
      {required this.recipientId,
      this.senderId,
      this.recentMessage,
      this.recentMessageDate});

  factory Conversation.fromJson(Map<String, dynamic> json) {
    return switch (json) {
      {
        'recipient_id': int recipientId,
        'sender_id': int? senderId,
        'recent_message_date': String? recentMessageDate,
        'recent_message': String? recentMessage,
      } =>
        Conversation(
            recipientId: recipientId,
            senderId: senderId,
            recentMessage: recentMessage,
            recentMessageDate: recentMessageDate),
      _ => throw const FormatException("Failed to load Message"),
    };
  }

  @override
  List get props => [recipientId, senderId, recentMessageDate, recentMessage];
}

Future<List<Message>> getUserMessages(int userId, String bearerToken) async {
  final apiPrefix = dotenv.env["API_URL"];
  final url = Uri.parse('$apiPrefix/chat/$userId');

  final response =
      await http.get(url, headers: {'Authorization': 'Bearer $bearerToken'});

  if (response.statusCode ~/ 100 != 2) {
    throw Exception("Failed to get user messages. Details: ${response.body}");
  }

  List<dynamic> jsonList = jsonDecode(response.body);
  List<Message> messages =
      jsonList.map((json) => Message.fromJson(json)).toList();

  return messages;
}

Future<void> sendMessage(
    int residentId, String bearerToken, String content) async {
  final apiPrefix = dotenv.env["API_URL"];
  final url = Uri.parse('$apiPrefix/chat');
  final body = jsonEncode({"content": content, "resident_id": residentId});

  final response = await http.post(url, body: body, headers: {
    'Content-Type': 'application/json',
    'Authorization': 'Bearer $bearerToken'
  });

  if (response.statusCode ~/ 100 != 2) {
    print(response.statusCode);
    throw Exception("Failed to send Message");
  }
}

Future<List<Conversation>> getConversations(String bearerToken) async {
  final apiPrefix = dotenv.env["API_URL"];
  final url = Uri.parse("$apiPrefix/chat/conversations");

  final response =
      await http.get(url, headers: {'Authorization': 'Bearer $bearerToken'});

  if (response.statusCode ~/ 100 != 2) {
    throw Exception("Failed to get conversations. Details: ${response.body}");
  }

  List<dynamic> jsonList = jsonDecode(response.body);
  List<Conversation> conversations =
      jsonList.map((json) => Conversation.fromJson(json)).toList();

  return conversations;
}
