import { render } from 'solid-js/web';
import { Router, Route } from "@solidjs/router";

import LandingPageRoutes from './landing_page/LandingPageRoutes';
import AccountRoutes from './accounts/AccountRoutes';
import SettingsRoutes from './settings/SettingsRoutes';

const App = () => {
  return (
    <Router>
      <Route path="/*">{LandingPageRoutes}</Route>
      <Route path="/*">{AccountRoutes}</Route>
      <Route path="/settings/*">{SettingsRoutes}</Route>
    </Router>
  )
}

/** @typedef { import('solid-js/web').MountableElement } MountableElement */
render(() => <App />, (/** @type MountableElement */ (document.getElementById('root'))) );
