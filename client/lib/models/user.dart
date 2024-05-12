import 'dart:convert';

import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:equatable/equatable.dart';
import 'package:http/http.dart' as http;
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

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
          "room_nr": int roomNumber,
          "username": String username,
        }
      } =>
        LoginResponse(
            bearerToken: bearerToken,
            user: User(
                firstName: convertToUtf8(firstName),
                lastName: convertToUtf8(lastName),
                roomNumber: roomNumber,
                username: convertToUtf8(username),
                role: UserRole.resident)),
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

  Map<String, dynamic> toJson() {
    return {
      'username': this.username,
      'first_name': this.firstName,
      'last_name': this.lastName,
      'room_nr': this.roomNumber,
      'role': this.role == UserRole.resident ? 'Resident' : 'Supervisor',
    };
  }

  Future<void> writeToStorage(String bearerToken) async {
    const storage = FlutterSecureStorage();

    await storage.write(key: "bearer_token", value: bearerToken);
    await storage.write(key: "user", value: jsonEncode(toJson()));
  }

  static Future<StorageData> getFromStorage() async {
    /*
    SharedPreferences prefs = await SharedPrefs.getInstance();

    String? bearerToken = prefs.getString('bearer_token');
    String? userJson = prefs.getString('user');
     */
    const storage = FlutterSecureStorage();

    String? bearerToken = await storage.read(key: "bearer_token");
    String? userJson = await storage.read(key: "user");

    User? user;

    if (userJson != null) {
      Map<String, dynamic> userMap = jsonDecode(userJson);
      user = User(
        username: userMap['username'],
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
    /*
    SharedPreferences prefs = await SharedPrefs.getInstance();

    prefs.remove('bearerToken');
    prefs.remove('user'); */

    const storage = FlutterSecureStorage();

    await storage.delete(key: "bearer_token");
    await storage.delete(key: "user");
  }

  static Future<LoginResponse> loginUser(String username, String password) async {
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

  const User(
      {this.username,
      this.firstName,
      this.lastName,
      this.roomNumber,
      this.role});

  @override
  List<Object?> get props => [username, firstName, lastName, roomNumber, role];
}

String convertToUtf8(String str) {
  var utf8runes = str.runes.toList();
  return utf8.decode(utf8runes);
}
