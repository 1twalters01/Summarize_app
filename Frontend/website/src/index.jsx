import { render } from 'solid-js/web';
import { Router, Route } from "@solidjs/router";

import LandingRoutes from './landing/LandingRoutes';
import AccountRoutes from './accounts/AccountRoutes';
import HomeRoutes from './home/HomeRoutes';
import SettingsRoutes from './settings/SettingsRoutes';
import SubscriptionsRoutes from './subscriptions/SubscriptionsRoutes';
import SummaryRoutes from './summary/SummaryRoutes';
import { EmailContextProvider } from './accounts/context/EmailContext';

const App = () => {
  return (
    <EmailContextProvider>
      <Router>
        <Route path="/*">{LandingRoutes}</Route>
        <Route path="/*">{AccountRoutes}</Route>
        <Route path="/*">{HomeRoutes}</Route>
        <Route path="/settings/*">{SettingsRoutes}</Route>
      </Router>
    </EmailContextProvider>
  )
}

/** @typedef { import('solid-js/web').MountableElement } MountableElement */
render(() => <App />, (/** @type MountableElement */ (document.getElementById('root'))) );
