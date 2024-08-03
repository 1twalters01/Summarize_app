import { Route } from '@solidjs/router';

// import Register from './pages/Register';
// import Login from './pages/Login';
import Logout from './pages/Logout';
import Activate from './pages/Activate';
import PasswordReset from './pages/PasswordReset';
import PasswordResetToken from './pages/PasswordResetToken';

import { lazy } from 'solid-js';
const Register = lazy(() => import('./pages/Register'));
const Login = lazy(() => import('./pages/Login'));
// const Logout = lazy(() => import('./Logout'));
// const Activate = lazy(() => import('./register/Activate'));
// const PasswordReset = lazy(() => import('./password_reset/PasswordReset'));
// const PasswordResetToken = lazy(() => import('./password_reset/PasswordResetToken'));

let AccountRoutes = () => {
  return (
    <>
      <Route path="register/" component={Register} />
      <Route path="login/" component={Login} />
      <Route path="password-reset/" component={PasswordReset} />
      <Route path="activate/:uidb64/:token/" component={Activate} />
      <Route
        path="password-reset/:uidb64/:token/"
        component={PasswordResetToken}
      />
      <Route path="logout/" component={Logout} />
    </>
  );
};

export default AccountRoutes;
