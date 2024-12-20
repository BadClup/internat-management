part of "chat_bloc.dart";

class ChatState extends Equatable {
  final List<Message>? messages;
  final List<Conversation>? conversations;
  final IOWebSocketChannel? wsChannel;
  final bool isLoading;
  final String? error;

  const ChatState({this.messages, this.conversations ,this.isLoading = false, this.error, this.wsChannel});

  @override
  List get props => [messages, conversations ,error, isLoading, wsChannel];
}
