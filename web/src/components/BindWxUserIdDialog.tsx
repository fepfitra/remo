import { useEffect, useState } from "react";
import { userService } from "../services";
import { showDialog } from "./Dialog";
import toastHelper from "./Toast";
import "../less/change-password-dialog.less";

interface Props extends DialogProps { }

const BindWxUserIdDialog: React.FC<Props> = ({ destroy }: Props) => {
  const [wxUserId, setWxUserId] = useState("");

  useEffect(() => {
    // do nth
  }, []);

  const handleCloseBtnClick = () => {
    destroy();
  };

  const handleWxUserIdChanged = (e: React.ChangeEvent<HTMLInputElement>) => {
    const text = e.target.value as string;
    setWxUserId(text);
  };

  const handleSaveBtnClick = async () => {
    if (wxUserId === "") {
      toastHelper.error("WeChat ID cannot be empty");
      return;
    }

    try {
      await userService.updateWxUserId(wxUserId);
      userService.doSignIn();
      toastHelper.info("Binding success!");
      handleCloseBtnClick();
    } catch (error: any) {
      toastHelper.error(error);
    }
  };

  return (
    <>
      <div className="dialog-header-container">
        <p className="title-text">Bind WeChat OpenID</p>
        <button className="btn close-btn" onClick={handleCloseBtnClick}>
          <img className="icon-img" src="/icons/close.svg" />
        </button>
      </div>
      <div className="dialog-content-container">
        <p className="tip-text">
          Follow WeChat Official Account 'Xiaotanxianshi', send any message to get <strong>OpenID</strong>.
        </p>
        <label className="form-label input-form-label">
          <span className={"normal-text " + (wxUserId === "" ? "" : "not-null")}>WeChat OpenID</span>
          <input type="text" value={wxUserId} onChange={handleWxUserIdChanged} />
        </label>
        <div className="btns-container">
          <span className="btn cancel-btn" onClick={handleCloseBtnClick}>
            Cancel
          </span>
          <span className="btn confirm-btn" onClick={handleSaveBtnClick}>
            Save
          </span>
        </div>
      </div>
    </>
  );
};

function showBindWxUserIdDialog() {
  showDialog(
    {
      className: "bind-wxid-dialog",
    },
    BindWxUserIdDialog
  );
}

export default showBindWxUserIdDialog;
