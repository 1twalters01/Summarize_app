import { useEmailContext } from '../../context/EmailContext';
import { handlePostEmail } from '../../functions/login/handlers';

/** @typedef { import('../../types/Props').LoginProps } Props */

/** @param {Props} props */
const LoginEmailForm = (props) => {
  const { email, setEmail } = useEmailContext();

  return (
    <>
      <form onSubmit={(e) => handlePostEmail(e, email(), props)}>
        <input
          type="email"
          placeholder="email"
          onInput={(e) => setEmail(e.target.value)}
          value={email()}
          required
        />
        <input type="submit" value="Continue" />
      </form>
    </>
  );
};

export default LoginEmailForm;
