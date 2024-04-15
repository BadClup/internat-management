import 'dart:convert';

import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:equatable/equatable.dart';
import 'package:http/http.dart' as http;

enum UserRole { supervisor, resident }

class User extends Equatable {
  final String? username;
  final String? firstName;
  final String? lastName;
  final int? roomNumber;
  final UserRole? role;

  const User(
      {this.username,
      this.firstName,
      this.lastName,
      this.roomNumber,
      this.role});

  @override
  List<Object?> get props => [username, firstName, lastName, roomNumber, role];
}

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
          "room_nr": int roomNumber,
          "username": String username,
        }
      } =>
        LoginResponse(
            bearerToken: bearerToken,
            user: User(
                firstName: firstName,
                lastName: lastName,
                roomNumber: roomNumber,
                username: username,
                role: UserRole.resident)),
      _ => throw const FormatException('Failed to load Login Response.'),
    };
  }
}

Future<LoginResponse> loginUser(String username, String password) async {
  final apiPrefix = dotenv.env["API_URL"];
  final url = Uri.parse('$apiPrefix/user/login');
  final body = jsonEncode({'username': username, 'password': password});

  final response = await http
      .post(url, body: body, headers: {'Content-Type': 'application/json'});

  if (response.statusCode < 400) {
    return LoginResponse.fromJson(
        jsonDecode(response.body) as Map<String, dynamic>);
  } else {
    throw Exception('Failed to login user');
  }
}
