//store.jsx

"use client";
import { combineReducers, configureStore } from "@reduxjs/toolkit";
import appReducer from "./storemodules/appSlice";
import { Provider } from "react-redux";

const rootReducer = combineReducers({
  app: appReducer,
  //add all your reducers here
});

export const store = configureStore({
  reducer: rootReducer,
});

export function ReduxProvider({ children }: { children: React.ReactNode }) {
  return <Provider store={store}>{children}</Provider>;
}

export type RootState = ReturnType<typeof store.getState>;

export type AppDispatch = typeof store.dispatch;
