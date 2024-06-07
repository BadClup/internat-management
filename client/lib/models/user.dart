import 'dart:convert';

import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:equatable/equatable.dart';
import 'package:http/http.dart' as http;
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

import '../utils/convert_to_utf_8.dart';

enum UserRole { supervisor, resident }

class LoginResponse {
  final String bearerToken;
  final User user;

  const LoginResponse({required this.bearerToken, required this.user});

  factory LoginResponse.fromJson(Map<String, dynamic> json) {
    return switch (json) {
      {
        'bearer_token': String bearerToken,
        'user': {
          "first_name": String firstName,
          "id": int id,
          "last_name": String lastName,
          "role": String role,
          "room_nr": int? roomNumber,
          "username": String username,
        }
      } =>
        LoginResponse(
            bearerToken: bearerToken,
            user: User(
                firstName: convertToUtf8(firstName),
                lastName: convertToUtf8(lastName),
                id: id,
                roomNumber: roomNumber,
                username: convertToUtf8(username),
                role: role.toLowerCase() == "resident" ? UserRole.resident : UserRole.supervisor)),
      _ => throw const FormatException('Failed to load Login Response.'),
    };
  }
}

class StorageData {
  final String? bearerToken;
  final User? user;

  StorageData({this.bearerToken, this.user});
}

class User extends Equatable {
  final String? username;
  final String? firstName;
  final String? lastName;
  final int? roomNumber;
  final UserRole? role;
  final int? id;

  Map<String, dynamic> toJson() {
    return {
      'username': username,
      'id': id,
      'first_name': firstName,
      'last_name': lastName,
      'room_nr': roomNumber,
      'role': role == UserRole.resident ? 'Resident' : 'Supervisor',
    };
  }

  Future<void> writeToStorage(String bearerToken) async {
    const storage = FlutterSecureStorage();

    await storage.write(key: "bearer_token", value: bearerToken);
    await storage.write(key: "user", value: jsonEncode(toJson()));
  }

  static Future<StorageData> getFromStorage() async {
    const storage = FlutterSecureStorage();

    String? bearerToken = await storage.read(key: "bearer_token");
    String? userJson = await storage.read(key: "user");

    User? user;

    if (userJson != null) {
      Map<String, dynamic> userMap = jsonDecode(userJson);
      user = User(
        username: userMap['username'],
        id: userMap['id'],
        firstName: userMap['first_name'],
        lastName: userMap['last_name'],
        roomNumber: userMap['room_nr'],
        role: userMap['role'] == 'Resident'
            ? UserRole.resident
            : UserRole.supervisor,
      );
    }

    return StorageData(bearerToken: bearerToken, user: user);
  }

  static Future<void> clearStorage() async {
    const storage = FlutterSecureStorage();

    await storage.delete(key: "bearer_token");
    await storage.delete(key: "user");
  }

  static Future<LoginResponse> loginUser(
      String username, String password) async {
    final apiPrefix = dotenv.env["API_URL"];
    final url = Uri.parse('$apiPrefix/user/login');
    final body = jsonEncode({'username': username, 'password': password});

    final response = await http
        .post(url, body: body, headers: {'Content-Type': 'application/json'});

    if (response.statusCode ~/ 100 != 2) {
      throw Exception('Failed to login user');
    }
    return LoginResponse.fromJson(
        jsonDecode(response.body) as Map<String, dynamic>);
  }

  const User(
      {this.username,
      this.id,
      this.firstName,
      this.lastName,
      this.roomNumber,
      this.role});

  @override
  List<Object?> get props =>
      [username, id, firstName, lastName, roomNumber, role];
}

