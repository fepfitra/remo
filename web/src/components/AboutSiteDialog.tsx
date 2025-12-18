import { showDialog } from "./Dialog";
import "../less/about-site-dialog.less";

interface Props extends DialogProps { }

const AboutSiteDialog: React.FC<Props> = ({ destroy }: Props) => {
  const handleCloseBtnClick = () => {
    destroy();
  };

  return (
    <>
      <div className="dialog-header-container">
        <p className="title-text">
          <span className="icon-text">🤠</span>About <b>Memos</b>
        </p>
        <button className="btn close-btn" onClick={handleCloseBtnClick}>
          <img className="icon-img" src="/icons/close.svg" />
        </button>
      </div>
      <div className="dialog-content-container">
        <p>A fragmented knowledge recording tool.</p>
        <br />
        <i>Why build this?</i>
        <ul>
          <li>
            Practice <strong>Zettelkasten Note-taking Method</strong>;
          </li>
          <li>Used for: 📅 Daily/Weekly Plans, 💡 Sudden Thoughts, 📕 Reading Reflections...</li>
          <li>Replaces the 'File Transfer Assistant' I often use on WeChat;</li>
          <li>Create a lightweight 'card' notebook of your own;</li>
        </ul>
        <br />
        <i>What are the features?</i>
        <ul>
          <li>
            ✨{" "}
            <a target="_blank" href="https://github.com/boojack/insmemo-web" rel="noreferrer">
              Open Source Project
            </a>
          </li>
          <li>😋 Exquisite and detailed visual style;</li>
          <li>📑 Excellent interactive logic experience;</li>
        </ul>
        <br />
        <a target="_blank" href="https://github.com/boojack/insmemo-web/discussions" rel="noreferrer">
          🤔 Feedback
        </a>
        <br />
        <p>Enjoy it and have fun~ </p>
        <hr />
        <p className="normal-text">
          Last updated on <span className="pre-text">2021/11/26 16:17:44</span> 🎉
        </p>
      </div>
    </>
  );
};

export default function showAboutSiteDialog(): void {
  showDialog(
    {
      className: "about-site-dialog",
    },
    AboutSiteDialog
  );
}
