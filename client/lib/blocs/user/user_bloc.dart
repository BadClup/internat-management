import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:internat_management/models/user.dart';

part "user_event.dart";

part 'user_state.dart';

class UserBloc extends Bloc<UserEvent, UserState> {
  UserBloc() : super(const UserState(user: User())) {
    on<LoginUser>((event, emit) async {
      try {
        final data = await loginUser(event.username, event.password);

        final user = data.user;

        emit(UserState(
            user: user,
            error: "", bearerToken: data.bearerToken));
      } catch (e) {
        emit(const UserState(user: User(), error: "Could not login user"));
      }
    });

    on<LogoutUser>((event, emit) {
      emit(const UserState(
        user: User(),
      ));
    });
  }
}
