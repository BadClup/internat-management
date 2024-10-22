import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:internat_management/models/chat.dart';
import 'package:web_socket_channel/io.dart';

part "chat_event.dart";

part "chat_state.dart";

class ChatBloc extends Bloc<ChatEvent, ChatState> {
  ChatBloc() : super(const ChatState()) {
    on<GetMessages>((event, emit) async {
      emit(const ChatState(isLoading: true, error: null));

      try {
        final data = await getUserMessages(event.userId, event.bearerToken);
        // print("Got messages!");
        emit(ChatState(messages: data, isLoading: false, error: null));
      } catch (e) {
        // print("error on getMessages: $e");
        emit(const ChatState(
            messages: null, isLoading: false, error: "Could not get messages"));
      }
    });

    on<SendMessage>((event, emit) async {
      emit(const ChatState(isLoading: true));

      try {
        await sendMessage(event.residentId, event.bearerToken, event.content);
        final data = await getUserMessages(event.residentId, event.bearerToken);

        emit(ChatState(messages: data, isLoading: false, error: null));
      } catch (e) {
        print("Error on SendMessage: $e");
        emit(ChatState(isLoading: false, error: "$e"));
      }
    });

    on<GetConversations>((event, emit) async {
      try {
        emit(const ChatState(isLoading: true, error: null));
        final data = await getConversations(event.bearerToken);
        emit(ChatState(isLoading: false, conversations: data));
      } catch (e) {
        print("Error on GetConversations: $e");
        emit(const ChatState(
            isLoading: false, error: "Nie udało się pobrać konwersacji"));
      }
    });

    on<ConnectToWebsocket>((event, emit) async {
      try {
        final channel =
            await connectToWebsocket(event.residentId, event.bearerToken);

        print("Connected to websocket");
        emit(ChatState(wsChannel: channel));
      } catch (e) {
        print("Error on ConnectToWebsocket: $e");
      }
    });

    on<ListenWebsocket>((event, emit) async {
      print("start listening websocket");

      final channel = event.channel;

      channel.stream.listen(
        (data) {
          print("New message: $data");
        },
        onError: (error) {
          print("Error in WebSocket: $error");
        },
        onDone: () {
          print("WebSocket stream closed");
        },
      );
    });

    on<GetMessagesAndConnectToWs>((event, emit) async {
      emit(const ChatState(isLoading: true, error: null));

      try {
        final data = await getUserMessages(event.residentId, event.bearerToken);
        final channel =
            await connectToWebsocket(event.residentId, event.bearerToken);
        emit(ChatState(
            messages: data, wsChannel: channel, isLoading: false, error: null));
      } catch (e) {
        emit(ChatState(isLoading: false, error: "$e"));
      }
    });
  }
}
