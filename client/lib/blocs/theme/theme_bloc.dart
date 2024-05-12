import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:internat_management/theme.dart';

part 'theme_event.dart';

part 'theme_state.dart';

class ThemeBloc extends Bloc<ThemeEvent, ThemeState> {
  ThemeBloc() : super(ThemeState(themeData: darkTheme)) {

    on<ChangeTheme>((event, emit) {
      emit(ThemeState(themeData: event.themeData));
    });
  }
}
