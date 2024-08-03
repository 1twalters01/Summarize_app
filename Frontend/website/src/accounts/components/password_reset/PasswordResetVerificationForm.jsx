import { createSignal } from 'solid-js';
import { handlePostVerification } from '../../functions/password_reset/handlers.js';

/** @typedef { import('../../types/Props.js').PasswordResetProps } Props */

/** @param {Props} props */
const PasswordResetVerificationForm = (props) => {
  const [code, setCode] = createSignal('');

  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode?.()}>
        x
      </button>
      <form onSubmit={(e) => handlePostVerification(e, code(), props)}>
        <input
          type="text"
          placeholder="token"
          onInput={(e) => setCode(e.target.value)}
          required
        />
        <input type="submit" value="Submit" />
      </form>
    </>
  );
};

export default PasswordResetVerificationForm;
