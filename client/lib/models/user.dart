import 'dart:convert';

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

Future loginUser(String username, String password) async {
  final url = Uri.parse('http://localhost:3000/user/login');
  final body = jsonEncode({'username': username, 'password': password});

  final response = await http
      .post(url, body: body, headers: {'Content-Type': 'application/json'});

  if (response.statusCode < 400) {
    final data = jsonDecode(response.body);
    print(data);
    return data;
  } else {
    throw Exception('Failed to login user');
  }
}
