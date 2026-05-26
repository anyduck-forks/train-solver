<section class="page">
  <div class="card">
    <p class="kicker">Page 4</p>
    <h1>Under The Hood</h1>
    <p>Preset 4 locks to the rear view, with higher scale and no spin.</p>
  </div>
</section>

<style>
  .page {
    min-height: 100vh;
    display: grid;
    place-items: center;
    padding: 10vh 8vw;
  }

  .card {
    max-width: 540px;
    padding: 32px 36px;
    border-radius: 24px;
    background: rgba(8, 12, 20, 0.76);
    box-shadow: 0 20px 60px rgba(3, 6, 12, 0.6);
    backdrop-filter: blur(12px);
  }

  .kicker {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.24em;
    margin: 0 0 12px;
    color: rgba(214, 225, 240, 0.75);
  }

  h1 {
    margin: 0 0 16px;
    font-size: clamp(2.2rem, 3vw, 3.2rem);
  }

  p {
    margin: 0;
    color: rgba(220, 232, 245, 0.82);
    line-height: 1.6;
  }
</style>
