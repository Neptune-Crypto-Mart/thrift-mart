export default function Home() {
  const features = [
    {
      title: "Pay with XLM or USDC",
      description: "Fast, low-fee transactions settled on the Stellar network.",
    },
    {
      title: "Escrow protection",
      description: "Funds are held safely until both sides confirm the trade.",
    },
    {
      title: "Global reach",
      description: "Buy and sell pre-loved items with anyone, anywhere.",
    },
  ];

  return (
    <div className="flex flex-1 flex-col items-center bg-background">
      <main className="flex w-full max-w-3xl flex-1 flex-col items-center justify-center gap-12 px-6 py-24 text-center">
        <div className="flex flex-col items-center gap-4">
          <h1 className="text-4xl font-semibold tracking-tight sm:text-5xl">
            Thrift Mart
          </h1>
          <p className="max-w-xl text-lg text-zinc-600 dark:text-zinc-400">
            A crypto-powered marketplace on Stellar for selling pre-loved items
            at affordable prices. Declutter smart, buy sustainably.
          </p>
        </div>

        <dl className="grid w-full gap-6 sm:grid-cols-3">
          {features.map((feature) => (
            <div
              key={feature.title}
              className="flex flex-col gap-2 rounded-xl border border-black/[.08] p-5 text-left dark:border-white/[.145]"
            >
              <dt className="font-medium">{feature.title}</dt>
              <dd className="text-sm text-zinc-600 dark:text-zinc-400">
                {feature.description}
              </dd>
            </div>
          ))}
        </dl>
      </main>
    </div>
  );
}
