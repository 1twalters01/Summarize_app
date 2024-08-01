import { createSignal, Switch, Match } from 'solid-js';

import Navbar from '../components/navbar';
import Headers from "../components/headers";
import Oauth2 from "../components/oauth";
import LoginEmailForm from '../components/login/LoginEmailForm';
import LoginPasswordForm from '../components/login/LoginPasswordForm';
import LoginTotpForm from '../components/login/LoginTotpForm';
import Footer from "../components/footer";

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */


const Login = () => {
  /** @readonly @enum {number} - Enum for mode values. */
  const modeOptions = {
    email: 0,
    password: 1,
    totp: 2,
  };

  const [mode, setMode] = createSignal(modeOptions.email);
  const emailMode = () => setMode(modeOptions.email);
  const passwordMode = () => setMode(modeOptions.password);
  const totpMode = () => setMode(modeOptions.totp);

  const subheader = "Log in to Summarize";
  const googleText = "Continue with Google";
  const appleText = "Continue with Apple";
  const guestText = "Continue as Guest";

  return (
    <>
      <Navbar />

      <Headers subheader={subheader} />

      <Oauth2 googleText={googleText} appleText={appleText} guestText={guestText} />

      <Switch>
        <Match when={mode() === modeOptions.email}>
          <LoginEmailForm passwordMode={passwordMode} />
        </Match>
        <Match when={mode() === modeOptions.password}>
          <LoginPasswordForm emailMode={emailMode} totpMode={totpMode} />
        </Match>
        <Match when={mode() === modeOptions.totp}>
          <LoginTotpForm emailMode={emailMode} />
        </Match>
      </Switch>

      <Footer />
    </>
  );
};

export default Login;


