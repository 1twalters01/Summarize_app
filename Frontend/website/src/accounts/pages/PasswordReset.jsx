import { createSignal, Switch, Match, lazy } from 'solid-js';

import Navbar from '../components/navbar';
import Headers from '../components/headers';
import Oauth2 from '../components/oauth';
import PasswordResetEmailForm from '../components/password_reset/PasswordResetEmailForm';
import PasswordResetVerificationForm from '../components/password_reset/PasswordResetVerificationForm';
import PasswordResetPasswordForm from '../components/password_reset/PasswordResetPasswordForm';
import Footer from '../components/footer';

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */

const PasswordReset = () => {
  /** @enum {number} Enum for mode values */
  const modeOptions = {
    email: 0,
    verify: 1,
    password: 2
  };

  const [mode, setMode] = createSignal(modeOptions.email);
  const emailMode = () => setMode(modeOptions.email);
  const verificationMode = () => setMode(modeOptions.verify);
  const detailsMode = () => setMode(modeOptions.password);

  const subheader = 'Reset your Summarize Password';
  const googleText = 'Log in with Google';
  const appleText = 'Log in with Apple';
  const guestText = 'Log in as Guest';

  return (
    <>
      <Navbar />

      <Headers subheader={subheader} />

      <Oauth2
        googleText={googleText}
        appleText={appleText}
        guestText={guestText}
      />

      <Switch>
        <Match when={modeOptions.email === mode()}>
          <PasswordResetEmailForm verificationMode={verificationMode} />
        </Match>
        <Match when={modeOptions.verify === mode()}>
          <PasswordResetVerificationForm
            emailMode={emailMode}
            passwordMode={detailsMode}
          />
        </Match>
        <Match when={modeOptions.password === mode()}>
          <PasswordResetPasswordForm emailMode={emailMode} />
        </Match>
      </Switch>

      <Footer />
    </>
  );
};

export default PasswordReset;
