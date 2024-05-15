"use client"

import { createSlice } from "@reduxjs/toolkit";
import { RootState } from "../store";
import { ConfigType, UserInfoType } from '@/interface';

const initialState = {
  isLogin: false,
  showLogIn: false,
  showSignUp: false,
  config: <ConfigType>{},
  showForget: false,
  showChangePassword: false,
  showChangeEmail: false,
  showChangePicture: false,
  userInfo: <UserInfoType>{}
};

export const appSlice = createSlice({
  name: "app",
  initialState,
  reducers: {
    changeIsLogin: (state, action) => {
      state.isLogin = action.payload
    },
    changeShowLogIn: (state, action) => {
      state.showLogIn = action.payload
    },
    changeShowSignUp: (state, action) => {
      state.showSignUp = action.payload
    },
    changeConfig: (state, action) => {
      state.config = action.payload
    },
    changeShowForget: (state, action) => {
      state.showForget = action.payload
    },
    changeUserInfo: (state, action) => {
      state.userInfo = action.payload
    },
    changeShowChangePassword: (state, action) => {
      state.showChangePassword = action.payload
    },
    changeShowChangeEmail: (state, action) => {
      state.showChangeEmail = action.payload
    },
    changeShowChangePicture: (state, action) => {
      state.showChangePicture = action.payload
    },
  },
});

export const { changeIsLogin, changeShowLogIn, changeShowSignUp, changeConfig, changeShowForget, changeUserInfo, changeShowChangePassword, changeShowChangeEmail, changeShowChangePicture } = appSlice.actions;

export default appSlice.reducer;

export const selectIsLogin = (state: RootState) => state.app.isLogin;
export const selectShowLogIn = (state: RootState) => state.app.showLogIn;
export const selectShowSignUp = (state: RootState) => state.app.showSignUp;
export const selectConfig = (state: RootState) => state.app.config;
export const selectShowForget = (state: RootState) => state.app.showForget;
export const selectUserInfo = (state: RootState) => state.app.userInfo;
export const selectShowChangePassword = (state: RootState) => state.app.showChangePassword;
export const selectShowChangeEmail = (state: RootState) => state.app.showChangeEmail;
export const selectShowChangePicture = (state: RootState) => state.app.showChangePicture;
