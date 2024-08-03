import { lazy } from 'solid-js';
import { Route } from '@solidjs/router';

const Landing = lazy(() => import('./pages/landing'));
const DownloadMobile = lazy(() => import('./pages/download/download_mobile'));
const DownloadDesktop = lazy(() => import('./pages/download/download_desktop'));
const DownloadWebClipper = lazy(
  () => import('./pages/download/download_web_clipper')
);
const Blog = lazy(() => import('./pages/community/blog'));
const Community = lazy(() => import('./pages/community/community'));
const Webinars = lazy(() => import('./pages/community/webinar'));
const Ai = lazy(() => import('./pages/products/ai'));
const Pricing = lazy(() => import('./pages/products/pricing'));
const Releases = lazy(() => import('./pages/products/releases'));
const Library = lazy(() => import('./pages/products/library'));
const Sync = lazy(() => import('./pages/products/sync'));
const Code = lazy(() => import('./pages/products/code'));
const AboutUs = lazy(() => import('./pages/company/about_us'));
const EmailUs = lazy(() => import('./pages/company/email_us'));
const Security = lazy(() => import('./pages/company/security'));
const CookieSettings = lazy(() => import('./pages/company/cookie_settings'));
const Terms = lazy(() => import('./pages/company/terms'));
const Privacy = lazy(() => import('./pages/company/privacy'));

/** @returns {import("solid-js/types/jsx").JSX.Element} */
const LandingRoutes = () => {
  return (
    <>
      <Route path="/" component={Landing} />
      <Route path="/downloads/mobile" component={DownloadMobile} />
      <Route path="/downloads/desktop" component={DownloadDesktop} />
      <Route path="/downloads/web-clipper" component={DownloadWebClipper} />
      <Route path="/blog" component={Blog} />
      <Route path="/community" component={Community} />
      <Route path="/webinars" component={Webinars} />
      <Route path="/ai" component={Ai} />
      <Route path="/pricing" component={Pricing} />
      <Route path="/releases" component={Releases} />
      <Route path="/library" component={Library} />
      <Route path="/sync" component={Sync} />
      <Route path="/code" component={Code} />
      <Route path="/about-us" component={AboutUs} />
      <Route path="/email-us" component={EmailUs} />
      <Route path="/security" component={Security} />
      <Route path="/cookie-settings" component={CookieSettings} />
      <Route path="/terms" component={Terms} />
      <Route path="/privacy" component={Privacy} />
    </>
  );
};

export default LandingRoutes;
