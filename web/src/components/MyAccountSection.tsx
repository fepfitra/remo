import { useContext, useState } from "react";
import appContext from "../stores/appContext";
import { userService } from "../services";
import utils from "../helpers/utils";
import { validate, ValidatorConfig } from "../helpers/validator";
import Only from "./common/OnlyWhen";
import toastHelper from "./Toast";
import showChangePasswordDialog from "./ChangePasswordDialog";
import showBindWxUserIdDialog from "./BindWxUserIdDialog";
import "../less/my-account-section.less";

const validateConfig: ValidatorConfig = {
  minLength: 4,
  maxLength: 24,
  noSpace: true,
  noChinese: true,
};

interface Props { }

const MyAccountSection: React.FC<Props> = () => {
  const { userState } = useContext(appContext);
  const user = userState.user as Model.User;
  const [username, setUsername] = useState<string>(user.username);
  const [showEditUsernameInputs, setShowEditUsernameInputs] = useState(false);
  const [showConfirmUnbindGithubBtn, setShowConfirmUnbindGithubBtn] = useState(false);
  const [showConfirmUnbindWxBtn, setShowConfirmUnbindWxBtn] = useState(false);

  const handleUsernameChanged = (e: React.ChangeEvent<HTMLInputElement>) => {
    const nextUsername = e.target.value as string;
    setUsername(nextUsername);
  };

  const handleConfirmEditUsernameBtnClick = async () => {
    if (user.username === "guest") {
      toastHelper.info("🈲 Do not change my username");
      return;
    }

    if (username === user.username) {
      setShowEditUsernameInputs(false);
      return;
    }

    const usernameValidResult = validate(username, validateConfig);
    if (!usernameValidResult.result) {
      toastHelper.error("Username " + usernameValidResult.reason);
      return;
    }

    try {
      const isUsable = await userService.checkUsernameUsable(username);

      if (!isUsable) {
        toastHelper.error("Username invalid");
        return;
      }

      await userService.updateUsername(username);
      await userService.doSignIn();
      setShowEditUsernameInputs(false);
      toastHelper.info("Update success~");
    } catch (error: any) {
      toastHelper.error(error.message);
    }
  };

  const handleChangePasswordBtnClick = () => {
    if (user.username === "guest") {
      toastHelper.info("🈲 Do not change my password");
      return;
    }

    showChangePasswordDialog();
  };

  const handleUnbindGithubBtnClick = async () => {
    if (showConfirmUnbindGithubBtn) {
      try {
        await userService.removeGithubName();
        await userService.doSignIn();
      } catch (error: any) {
        toastHelper.error(error.message);
      }
      setShowConfirmUnbindGithubBtn(false);
    } else {
      setShowConfirmUnbindGithubBtn(true);
    }
  };

  const handleUnbindWxBtnClick = async () => {
    if (showConfirmUnbindWxBtn) {
      try {
        await userService.updateWxUserId("");
        await userService.doSignIn();
      } catch (error: any) {
        toastHelper.error(error.message);
      }
      setShowConfirmUnbindWxBtn(false);
    } else {
      setShowConfirmUnbindWxBtn(true);
    }
  };

  const handlePreventDefault = (e: React.MouseEvent) => {
    e.preventDefault();
  };

  return (
    <>
      <div className="section-container account-section-container">
        <p className="title-text">Basic Information</p>
        <label className="form-label input-form-label">
          <span className="normal-text">ID：</span>
          <span className="normal-text">{user.id}</span>
        </label>
        <label className="form-label input-form-label">
          <span className="normal-text">Created at:</span>
          <span className="normal-text">{utils.getDateString(user.createdAt)}</span>
        </label>
        <label className="form-label input-form-label username-label">
          <span className="normal-text">Username:</span>
          <input
            type="text"
            readOnly={!showEditUsernameInputs}
            value={username}
            onClick={() => {
              setShowEditUsernameInputs(true);
            }}
            onChange={handleUsernameChanged}
          />
          <div className="btns-container" onClick={handlePreventDefault}>
            <span className={"btn confirm-btn " + (showEditUsernameInputs ? "" : "hidden")} onClick={handleConfirmEditUsernameBtnClick}>
              Save
            </span>
            <span
              className={"btn cancel-btn " + (showEditUsernameInputs ? "" : "hidden")}
              onClick={() => {
                setUsername(user.username);
                setShowEditUsernameInputs(false);
              }}
            >
              Cancel
            </span>
          </div>
        </label>
        <label className="form-label password-label">
          <span className="normal-text">Password:</span>
          <span className="btn" onClick={handleChangePasswordBtnClick}>
            Change Password
          </span>
        </label>
      </div>
      {/* Account Binding Settings: only can use for domain: memos.justsven.top */}
      <Only when={window.location.origin.includes("justsven.top")}>
        <div className="section-container connect-section-container">
          <p className="title-text">Linked Accounts</p>
          <label className="form-label input-form-label">
            <span className="normal-text">WeChat OpenID:</span>
            {user.wxUserId ? (
              <>
                <span className="value-text">************</span>
                <span
                  className={`btn-text unbind-btn ${showConfirmUnbindWxBtn ? "final-confirm" : ""}`}
                  onMouseLeave={() => setShowConfirmUnbindWxBtn(false)}
                  onClick={handleUnbindWxBtnClick}
                >
                  {showConfirmUnbindWxBtn ? "Confirm Unbind!" : "Unbind"}
                </span>
              </>
            ) : (
              <>
                <span className="value-text">Null</span>
                <span
                  className="btn-text bind-btn"
                  onClick={() => {
                    showBindWxUserIdDialog();
                  }}
                >
                  Bind ID
                </span>
              </>
            )}
          </label>
          <label className="form-label input-form-label">
            <span className="normal-text">GitHub：</span>
            {user.githubName ? (
              <>
                <a className="value-text" href={"https://github.com/" + user.githubName}>
                  {user.githubName}
                </a>
                <span
                  className={`btn-text unbind-btn ${showConfirmUnbindGithubBtn ? "final-confirm" : ""}`}
                  onMouseLeave={() => setShowConfirmUnbindGithubBtn(false)}
                  onClick={handleUnbindGithubBtnClick}
                >
                  {showConfirmUnbindGithubBtn ? "Confirm Unbind!" : "Unbind"}
                </span>
              </>
            ) : (
              <>
                <span className="value-text">Null</span>
                <a
                  className="btn-text link-btn"
                  href="https://github.com/login/oauth/authorize?client_id=187ba36888f152b06612&scope=read:user,gist"
                >
                  Go to Bind
                </a>
              </>
            )}
          </label>
        </div>
      </Only>
    </>
  );
};

export default MyAccountSection;
