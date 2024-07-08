import { createSignal, Switch, Match } from 'solid-js';
import Navbar from '../components/navbar';
import Headers from "../components/login/headers";
import Oauth2 from "../components/login/oauth";

import LoginEmailFormFragment from '../components/login/login_email_form_fragment';
import LoginPasswordFormFragment from '../components/login/login_password_form_fragment';
import LoginTotpFormFragment from '../components/login/login_totp_form_fragment';

import Footer from "../components/login/footer";
import { EmailContextProvider } from '../context/EmailContext';

import { useContext } from 'solid-js';
import { EmailContext } from '../context/EmailContext';

/** 
  * @template T
  * @typedef { import('solid-js').Accessor<T> } Accessor
*/

/**
  * @template T
  * @typedef { import('solid-js').Setter<T> } Setter
*/

/**
  * @template Y
  * @typedef { import('solid-js').Signal<Y> } Signal
*/

    


const Login = () => {
  /** Enum for mode values.
    * @readonly
    * @enum {number}
  */
  var modeOptions = {
    email: 0,
    password: 1,
    totp: 2,
  };

  /** @type {Signal<modeOptions>} */
  const [mode, setMode] = createSignal(modeOptions.email);

  const subheader = "Login to Summarize";

  const emailMode = () => {
    setMode(modeOptions.email);
  };

  const passwordMode = () => {
    setMode(modeOptions.password);
  };

  const totpMode = () => {
    setMode(modeOptions.totp);
  };

  return (
    <EmailContextProvider>
      <Navbar />

      <Headers subheader={subheader} />

      <Oauth2 />

      <Switch>
        <Match when={mode() === modeOptions.email}>
            <LoginEmailFormFragment passwordMode={passwordMode} />
        </Match>
        <Match when={mode() === modeOptions.password}>
          <LoginPasswordFormFragment emailMode={emailMode} totpMode={totpMode} />
        </Match>
        <Match when={mode() === modeOptions.totp}>
          <LoginTotpFormFragment emailMode={emailMode} />
        </Match>
      </Switch>

      <Footer />
    </EmailContextProvider>
  );
};

export default Login;


