import { createContext, createSignal } from "solid-js";

export const EmailContext = createContext();

export function EmailContextProvider(props) {
  const [email, setEmail] = createSignal(props.email || "");

  return (
    <EmailContext.Provider value={{email, setEmail}}>
      {props.children}
    </EmailContext.Provider>
  )
}

