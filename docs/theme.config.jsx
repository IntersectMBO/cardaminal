export default {
  logo: (
    <h1 className="font-bold text-4xl md:text-4xl lg:text-5xl">Cardaminal</h1>
  ),
  project: {
    link: "https://github.com/txpipe/cardaminal",
  },
  chat: {
    link: "https://discord.gg/Vc3x8N9nz2",
  },
  footer: {
    text: "Cardaminal",
  },
  nextThemes: {
    defaultTheme: "dark",
  },
  docsRepositoryBase: "https://github.com/txpipe/cardaminal/tree/main/docs",
  useNextSeoProps() {
    return {
      titleTemplate: "%s – Cardaminal",
      description:
        "Cardaminal is a CLI-based Cardano wallet tailored for power-users and developers",
      siteName: "Cardaminal",
    };
  },
};
