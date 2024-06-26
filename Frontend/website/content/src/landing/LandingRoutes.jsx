import { lazy } from 'solid-js';
import { Route } from '@solidjs/router';

const Landing = lazy(() => import('./pages/landing'));
const Pricing = lazy(() => import('./pages/pricing'));
const DownloadMobile = lazy(() => import('./pages/download/download_mobile'));
const DownloadDesktop = lazy(() => import('./pages/download/download_desktop'));
const DownloadWebClipper = lazy(() => import('./pages/download/download_web_clipper'));

/** @returns {import("solid-js/types/jsx").JSX.Element} */
const LandingRoutes = () => {
  return (
    <>
      <Route path="/" component={Landing} />
      <Route path="/pricing" component={Pricing} />
      <Route path="/downloads/mobile" component={DownloadMobile} />
      <Route path="/downloads/desktop" component={DownloadDesktop} />
      <Route path="/downloads/web-clipper" component={DownloadWebClipper} />
    </>
  )
};

export default LandingRoutes;

