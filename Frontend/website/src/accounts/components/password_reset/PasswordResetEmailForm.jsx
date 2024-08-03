import { useEmailContext } from '../../context/EmailContext';
import { handlePostEmail } from '../../functions/password_reset/handlers.js';

/** @typedef { import('../../types/Props.js').PasswordResetProps } Props */

/** @param {Props} props */
const PasswordResetEmailForm = (props) => {
  const { email, setEmail } = useEmailContext();

  return (
    <form onSubmit={(e) => handlePostEmail(e, email, props)}>
      <input
        type="email"
        placeholder="email"
        onInput={(e) => setEmail(e.target.value)}
        value={email()}
        required
      />
      <input type="submit" value="Continue" />
    </form>
  );
};

export default PasswordResetEmailForm;
