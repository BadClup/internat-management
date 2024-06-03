import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:internat_management/models/theme.dart';

part 'theme_event.dart';

part 'theme_state.dart';

class ThemeBloc extends Bloc<ThemeEvent, ThemeState> {
  ThemeBloc() : super(ThemeState(themeData: AppTheme.darkTheme)) {
    on<InitTheme>((event, emit) async {
      final theme = await AppTheme.getTheme();

      if (theme != null && theme != state.themeData) {
        emit(ThemeState(themeData: theme));
      }
    });

    on<ChangeTheme>((event, emit) async {
      await AppTheme.saveTheme(event.themeData);
      emit(ThemeState(themeData: event.themeData));
    });
  }
}
