import { lazy } from 'solid-js';
import { Router, Route } from '@solidjs/router';

const ChangeEmail = lazy(() => import('./ChangeEmail'));
const ChangeTheme = lazy(() => import('./ChangeTheme'));
const ChangePassword = lazy(() => import('./ChangePassword'));
const ChangeUsername = lazy(() => import('./ChangeUsername'));
const CloseAccount = lazy(() => import('./CloseAccount'));
const TwoFactorAuth = lazy(() => import('./TwoFactorAuth'));

let AccountRoutes = () => {
  return (
    <Router>
      <Route path="/settings/change-email/" component={ChangeEmail} />
      <Route path="/settings/change-theme/" component={ChangeTheme} />
      <Route path="/settings/change-password/" component={ChangePassword} />
      <Route path="/settings/change-username/" component={ChangeUsername} />
      <Route path="/settings/close-account" component={CloseAccount} />
      <Route path="/settings/two-factor-auth/" component={TwoFactorAuth} />
    </Router>
  )
};

export default AccountRoutes;

