part of "chat_bloc.dart";

abstract class ChatEvent extends Equatable {
  const ChatEvent();

  @override
  List<Object> get props => [];
}

class GetMessages extends ChatEvent {
  final int userId;
  final String bearerToken;

  const GetMessages({required this.userId, required this.bearerToken});

  @override
  List<Object> get props => [userId, bearerToken];
}

class SendMessage extends ChatEvent {
  final int residentId;
  final String bearerToken;
  final String content;

  const SendMessage(
      {required this.content,
      required this.bearerToken,
      required this.residentId});

  @override
  List<Object> get props => [content];
}

class GetConversations extends ChatEvent {
  final String bearerToken;

  const GetConversations({required this.bearerToken});

  @override
  List<Object> get props => [bearerToken];
}
