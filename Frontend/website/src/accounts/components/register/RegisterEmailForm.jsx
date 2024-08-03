import { useEmailContext } from '../../context/EmailContext';
import { handlePostEmail } from '../../functions/register/handlers.js';

/** @typedef { import ('../../types/Props').RegisterProps } Props */


/** @param {Props} props */
const RegisterEmailForm = (props) => {
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

export default RegisterEmailForm;
