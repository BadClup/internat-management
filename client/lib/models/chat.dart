import 'dart:convert';
import 'dart:io';

import 'package:equatable/equatable.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:http/http.dart' as http;
import 'package:internat_management/utils/convert_to_utf_8.dart';
import 'package:web_socket_channel/io.dart';

extension StatusExtension on http.Response {
  bool isStatusOk() {
    return statusCode ~/ 100 == 2;
  }
}

class ConversationUser extends Equatable {
  final int id;
  final String firstName;
  final String lastName;

  const ConversationUser(
      {required this.id, required this.firstName, required this.lastName});

  factory ConversationUser.fromJson(Map<String, dynamic> json) {
    return ConversationUser(
        id: json['id'],
        firstName: convertToUtf8(json['first_name']),
        lastName: convertToUtf8(json['last_name']));
  }

  @override
  List get props => [id, firstName, lastName];
}

class Message extends Equatable {
  final String content;
  final int id;
  final ConversationUser recipient;
  final ConversationUser sender;
  final String createdAt;

  const Message(
      {required this.recipient,
      required this.sender,
      required this.createdAt,
      required this.id,
      required this.content});

  factory Message.fromJson(Map<String, dynamic> json) {
    return switch (json) {
      {
        'content': String content,
        'id': int id,
        'recipient': Map<String, dynamic> recipient,
        'sender': Map<String, dynamic> sender,
        'created_at': String createdAt,
      } =>
        Message(
            content: convertToUtf8(content),
            id: id,
            createdAt: createdAt,
            recipient: ConversationUser.fromJson(recipient),
            sender: ConversationUser.fromJson(sender)),
      _ => throw const FormatException("Failed to load Message"),
    };
  }

  @override
  List get props => [content, id, recipient, sender, createdAt];
}

class Conversation extends Equatable {
  final ConversationUser recipient;
  final ConversationUser? sender;
  final String? recentMessageDate;
  final String? recentMessage;

  const Conversation(
      {required this.recipient,
      this.sender,
      this.recentMessage,
      this.recentMessageDate});

  factory Conversation.fromJson(Map<String, dynamic> json) {
    return switch (json) {
      {
        'recipient': Map<String, dynamic> recipient,
        'sender': Map<String, dynamic>? sender,
        'recent_message_date': String? recentMessageDate,
        'recent_message': String? recentMessage,
      } =>
        Conversation(
            recipient: ConversationUser.fromJson(recipient),
            sender: sender != null ? ConversationUser.fromJson(sender) : null,
            recentMessage: recentMessage != null ? convertToUtf8(recentMessage) : null,
            recentMessageDate: recentMessageDate),
      _ => throw const FormatException("Failed to load Message"),
    };
  }

  @override
  List get props => [recipient, sender, recentMessageDate, recentMessage];
}

Future<List<Message>> getUserMessages(int userId, String bearerToken) async {
  final apiPrefix = dotenv.env["API_URL"];
  final url = Uri.parse('$apiPrefix/chat/$userId');

  final response =
      await http.get(url, headers: {'Authorization': 'Bearer $bearerToken'});

  if (!response.isStatusOk()) {
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

  if (!response.isStatusOk()) {
    throw Exception("Failed to send Message");
  }
}

Future<List<Conversation>> getConversations(String bearerToken) async {
  final apiPrefix = dotenv.env["API_URL"];
  final url = Uri.parse("$apiPrefix/chat/conversations");

  final response =
      await http.get(url, headers: {'Authorization': 'Bearer $bearerToken'});

  if (!response.isStatusOk()) {
    throw Exception("Failed to get conversations. Details: ${response.body}");
  }

  List<dynamic> jsonList = jsonDecode(response.body);
  List<Conversation> conversations =
      jsonList.map((json) => Conversation.fromJson(json)).toList();

  return conversations;
}

Future<IOWebSocketChannel> connectToWebsocket(
    int residentId, String bearerToken) async {
  final apiPrefix = dotenv.env["API_URL"];
  final webSocketKey = dotenv.env["WEB_SOCKET_KEY"];
  final url = Uri.parse("$apiPrefix/chat/ws");

  final httpClientRequest = await HttpClient().getUrl(url);

  httpClientRequest.headers.set("Authorization", "Bearer $bearerToken");
  httpClientRequest.headers.set("resident-id", residentId);
  httpClientRequest.headers.set("Connection", "Upgrade");
  httpClientRequest.headers.set("Upgrade", "websocket");
  httpClientRequest.headers
      .set("Sec-WebSocket-Key", base64.encode(utf8.encode(webSocketKey!)));
  httpClientRequest.headers.set("Sec-WebSocket-Version", "13");

  final httpClientResponse = await httpClientRequest.close();
  if (httpClientResponse.statusCode != HttpStatus.switchingProtocols) {
    throw Exception(
        'Failed to connect to WebSocket: ${httpClientResponse.statusCode} ${httpClientResponse.reasonPhrase}');
  }

  final protocol = httpClientResponse.headers.value('sec-websocket-protocol');

  final socket = await httpClientResponse.detachSocket();
  final webSocket = WebSocket.fromUpgradedSocket(socket,
      protocol: protocol, serverSide: false);

  return IOWebSocketChannel(webSocket);
}
