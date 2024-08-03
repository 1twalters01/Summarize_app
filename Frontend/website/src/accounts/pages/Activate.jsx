import { useParams } from '@solidjs/router';

const Activate = () => {
  const { uidb64, token } = useParams();
  console.log(uidb64);
  console.log(token);

  return (
    <>
      <h1>Activate</h1>
      <p>uidb64: {uidb64}</p>
      <p>token: {token}</p>
    </>
  );
};

export default Activate;
