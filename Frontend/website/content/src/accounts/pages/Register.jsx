import { createSignal, Switch, Match } from 'solid-js';

import Navbar from '../components/navbar';
import Headers from "../components/headers";
import Oauth2 from "../components/oauth";
import RegisterEmailForm from "../components/register/RegisterEmailForm";
import RegisterVerificationForm from "../components/register/RegisterVerificationForm";
import RegisterDetailsForm from "../components/register/RegisterDetailsForm";
import Footer from "../components/footer";

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */


const Register = () => {
  /** @readonly @enum {number} - Enum for mode values. */
  const modeOptions = {
    email: 0,
    verification: 1,
    details: 2,
  };

  const [mode, setMode] = createSignal(modeOptions.email);
  const emailMode = () => setMode(modeOptions.email);
  const verificationMode = () => setMode(modeOptions.verification);
  const detailsMode = () => setMode(modeOptions.details);

  const subheader = "Sign up for Summarize";
  const googleText = "Continue with Google";
  const appleText = "Continue with Apple";
  const guestText = "Continue as Guest";

  return (
    <>
      <Navbar />

      <Headers subheader={subheader} />

      <Oauth2 googleText={googleText} appleText={appleText} guestText={guestText} />

      <Switch>
        <Match when={modeOptions.email === mode()}>
          <RegisterEmailForm verificationMode={verificationMode} />
        </Match>
        <Match when={modeOptions.verification === mode()}>
          <RegisterVerificationForm emailMode={emailMode} detailsMode={detailsMode} />
        </Match>
        <Match when={modeOptions.details === mode()}>
          <RegisterDetailsForm emailMode={emailMode} />
        </Match>
      </Switch>

      <Footer />
    </>
  );
};

export default Register;
