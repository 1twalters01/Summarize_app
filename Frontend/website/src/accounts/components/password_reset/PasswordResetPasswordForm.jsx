import { createSignal } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { handlePostPasswords } from '../../functions/password_reset/handlers.js';

/** @typedef { import('../../types/Props.js').PasswordResetProps } Props */

/** @param {Props} props */
const PasswordResetPasswordForm = (props) => {
  const [password, setPassword] = createSignal('');
  const [passwordConfirmation, setPasswordConfirmation] = createSignal('');
  const navigate = useNavigate();

  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode?.()}>
        x
      </button>

      <form onSubmit={(e) => handlePostPasswords(e, password(), passwordConfirmation(), navigate)}>
        <input
          type="password"
          placeholder="password confirmation"
          onInput={(e) => setPassword(e.target.value)}
          required
        />
        <input
          type="password"
          placeholder="password confirmation"
          onInput={(e) => setPasswordConfirmation(e.target.value)}
          required
        />
        <input type="submit" value="Submit" />
      </form>
    </>
  );
};

export default PasswordResetPasswordForm;
