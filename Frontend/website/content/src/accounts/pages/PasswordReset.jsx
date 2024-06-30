import { createSignal, Switch, Match, lazy } from 'solid-js';
import Navbar from '../components/navbar';
// const Navbar = lazy(() => import ('../navbar'));
const PasswordResetEmailForm = lazy(() => import('../fragments/password_reset/PasswordResetEmailForm'));
const PasswordResetVerificationForm = lazy(() => import('../fragments/password_reset/PasswordResetVerificationForm'));
const PasswordResetPasswordForm = lazy(() => import('../fragments/password_reset/PasswordResetPasswordForm'));

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


/** Enum for mode values.
  * @readonly
  * @enum {number}
  */
var modeOptions = {
  email: 0,
  verify: 1,
  password: 2,
};


const PasswordReset = () => {
  /** @type {Signal<modeOptions>} */
  const [mode, setMode] = createSignal(modeOptions.email);
  console.log(mode());

  // const emailMode = () => {
  //   setMode(modeOptions.email);
  // };

  const verificationMode = () => {
    setMode(modeOptions.verify);
  };

  const detailsMode = () => {
    setMode(modeOptions.password);
  };

  return (
    <>
      <Navbar />

      <h1>PasswordReset</h1>

      <Switch>
        <Match when={modeOptions.email === mode()}>
          <PasswordResetEmailForm verificationMode={verificationMode} />
        </Match>
        <Match when={modeOptions.verify === mode()}>
          <PasswordResetVerificationForm passwordMode={detailsMode} />
        </Match>
        <Match when={modeOptions.password === mode()}>
          <PasswordResetPasswordForm />
        </Match>
      </Switch>
    </>
  );
};

export default PasswordReset;



