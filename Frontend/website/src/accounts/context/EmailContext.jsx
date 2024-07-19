import { createContext, createSignal, useContext } from "solid-js";

export const EmailContext = createContext();


/** @typedef {Object} props
  * @property {Element} children - Child elements
*/

/** @param {props} props */ 
export function EmailContextProvider(props) {
  const [email, setEmail] = createSignal("");

  return (
    <EmailContext.Provider value={{email, setEmail}}>
      {props.children}
    </EmailContext.Provider>
  )
}

export function useEmailContext() {
  return useContext(EmailContext);
}

