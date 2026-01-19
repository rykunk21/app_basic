Top-Level Sections (Pages / Sections)

Your mobile-first, one-page portfolio (from our earlier plan) has these sections:

Hero – Eye-catching intro, headline + subheadline + primary CTA

Services / What I Offer – List of website services

Example Site / Portfolio – Show live example or screenshots

Process / How It Works – Steps to working with you

Target Audience / Who This Is For – Optional cards showing client types

About / Local Credibility – Short bio

Contact – Form, email link, CTA

Footer – Name, location, email

2️⃣ Components per Section
2.1 Hero

Hero – wrapper, background, spacing

Headline – large text with “colored pencil” effect

SubHeadline – secondary text

CTAButton – primary button

Optional: BackgroundIllustration – subtle pencil style shapes

2.2 Services

Services – section wrapper

ServiceCard – individual service item

ServiceList – map of ServiceCards

2.3 Example Site / Portfolio

Portfolio – section wrapper

PortfolioCard – single project

PortfolioGrid – grid layout for multiple projects

2.4 Process

Process – wrapper

ProcessStep – numbered step (icon + text)

ProcessList – sequence of steps

2.5 Target Audience

Audience – wrapper

AudienceCard – single client type

AudienceGrid – optional grid layout

2.6 About

About – wrapper

BioText – short paragraph with pencil effect

ProfileImage – optional

2.7 Contact

Contact – wrapper

ContactForm – form fields + submit button

ContactCTA – email / phone link

FormField – reusable input component

2.8 Footer

Footer – wrapper

FooterLinks – links / info

3️⃣ Styling Plan (Colored Pencil Feel + Tailwind)

Tailwind provides spacing, typography, and layout utilities.

Colored pencil effect can be achieved with:

Outline or text-shadow classes (Tailwind shadow-text custom)

Background gradients with subtle noise patterns

Borders / rounded elements with light sketch effect

Class management in Yew:

classes!() macro for dynamic class toggling

Can define reusable tailwind class strings per component for consistency

Example:

let headline_class = classes!(
"text-4xl font-bold",
"text-yellow-600",
"drop-shadow-md", // simulates pencil texture
"tracking-wide"
);

Animations / hover effects:

Use Tailwind transitions: transition-all duration-300 ease-out

Optional: custom keyframes for bounce, fade, slide

4️⃣ Final Component List

Wrapper / components:

Hero

Services

Portfolio

Process

Audience

About

Contact

Footer

Content / reusable components:

Headline

SubHeadline

CTAButton

BackgroundIllustration

ServiceCard

ServiceList

PortfolioCard

PortfolioGrid

ProcessStep

ProcessList

AudienceCard

AudienceGrid

BioText

ProfileImage

ContactForm

ContactCTA

FormField

FooterLinks

Optional helpers:

AnimatedDiv / AnimationWrapper (for transitions or spring effects)

✅ Next steps workflow for you:

Use Tailwind React components as templates

Translate each React component to Yew using the html! macro + classes!()

Organize files in components/ directory:

components/
hero.rs
services.rs
portfolio.rs
process.rs
audience.rs
about.rs
contact.rs
footer.rs
common/
cta_button.rs
card.rs
form_field.rs
animated_div.rs

Wire top-level components in App like:

html! {
<>
<Hero/>
<Services/>
<Portfolio/>
<Process/>
<Audience/>
<About/>
<Contact/>

<Footer/>
</>
}
