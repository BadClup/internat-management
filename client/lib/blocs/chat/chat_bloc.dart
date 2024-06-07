import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:internat_management/models/chat.dart';

part "chat_event.dart";
part "chat_state.dart";

class ChatBloc extends Bloc<ChatEvent, ChatState> {
  ChatBloc() : super(const ChatState()) {

    on<GetMessages>((event, emit) async {

        emit(const ChatState(isLoading: true, error: null));

        try {
          final data = await getUserMessages(event.userId, event.bearerToken);

          emit(ChatState(messages: data, isLoading: false, error: null));

        } catch(e) {
          print("error on getMessages: $e");
          emit(const ChatState(messages: null, isLoading: false, error: "Could not get messages"));
        }
    });

    on<SendMessage>((event, emit) async {

      emit(const ChatState(isLoading: true));

      try {
        await sendMessage(event.residentId, event.bearerToken, event.content);

        final data = await getUserMessages(event.residentId, event.bearerToken);

        emit(ChatState(messages: data, isLoading: false, error: null));

      } catch(e) {
        print("Error on SendMessage: $e");
        emit(ChatState(isLoading: false, error: "$e"));
      }
    });

    on<GetConversations>((event, emit) async {

      try {

        emit(const ChatState(isLoading: true, error: null));
        final data = await getConversations(event.bearerToken);
        emit(ChatState(isLoading: false, conversations: data));

      } catch(e) {
        print("Error on GetConversations: $e");
        emit(const ChatState(isLoading: false, error: "Nie udało się pobrać konwersacji"));
      }
    });
  }
}