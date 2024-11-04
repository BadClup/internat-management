import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter/foundation.dart';
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
        emit(ChatState(messages: data, isLoading: false, error: null));
      } catch (e) {
        emit(const ChatState(
            messages: null,
            isLoading: false,
            error: "Nie udało się pobrać wiadomości"));
      }
    });

    on<SendMessage>((event, emit) async {
      emit(const ChatState(isLoading: true));

      try {
        await sendMessage(event.residentId, event.bearerToken, event.content);
        final data = await getUserMessages(event.residentId, event.bearerToken);

        emit(ChatState(messages: data, isLoading: false, error: null));
      } catch (e) {
        emit(const ChatState(
            isLoading: false, error: "Nie udało się wysłać wiadomości"));
      }
    });

    on<GetConversations>((event, emit) async {
      try {
        emit(const ChatState(isLoading: true, error: null));
        final data = await getConversations(event.bearerToken);
        print(data);
        emit(ChatState(isLoading: false, conversations: data));
      } catch (e) {
        print(e);
        emit(const ChatState(
            isLoading: false, error: "Nie udało się pobrać konwersacji"));
      }
    });

    on<ListenWebsocket>((event, emit) async {
      final channel = event.channel;

      channel.stream.listen(
        (data) {
          if (kDebugMode) {
            print("New message: $data");
          }
        },
        onError: (error) {
          if (kDebugMode) {
            print("Error in WebSocket: $error");
          }
        },
        onDone: () {
          if (kDebugMode) {
            print("WebSocket stream closed");
          }
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
        emit(const ChatState(
            isLoading: false, error: "Nie udało się pobrać wiadomości"));
      }
    });
  }
}
