part of "chat_bloc.dart";

class ChatState extends Equatable {
  final List<Message>? messages;
  final bool isLoading;
  final String? error;

  const ChatState({this.messages, this.isLoading = false, this.error});

  @override
  List get props => [messages, error, isLoading];
}
