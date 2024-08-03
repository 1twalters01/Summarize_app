import { useParams } from '@solidjs/router';

const PasswordResetToken = () => {
  const { uidb64, token } = useParams();
  console.log(uidb64);
  console.log(token);

  return (
    <>
      <h1>Password Reset</h1>
      <p>uidb64: {uidb64}</p>
      <p>token: {token}</p>
    </>
  );
};

export default PasswordResetToken;
