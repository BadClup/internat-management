import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:internat_management/models/user.dart';

part "user_event.dart";

part 'user_state.dart';

class UserBloc extends Bloc<UserEvent, UserState> {
  UserBloc() : super(const UserState(user: User(), isAuth: false)) {
    on<LoginUser>((event, emit) async {
      try {
        final data = await loginUser(event.username, event.password);

        emit(UserState(
            user: User(username: event.username, role: UserRole.resident),
            isAuth: true));
      } catch (e) {
        emit(const UserState(
            user: User(), isAuth: false, error: "Could not login user"));
      }
    });

    on<LogoutUser>((event, emit) {
      emit(const UserState(user: User(), isAuth: false));
    });
  }
}
