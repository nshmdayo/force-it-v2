package application3;

import application.Main;
import javafx.beans.value.ObservableValue;
import javafx.scene.Group;
import javafx.scene.PerspectiveCamera;
import javafx.scene.Scene;
import javafx.scene.control.Label;
import javafx.scene.control.Slider;
import javafx.stage.Stage;

public class ParamWindow extends Stage{

	private double x = Main.CAMERA_X, y = Main.CAMERA_Y, z = Main.CAMERA_Z;

	public ParamWindow(PerspectiveCamera camera) {

		final Slider sx = new Slider(-1000, 1000, x);
	    final Slider sy = new Slider(-1000, 1000, y);
	    final Slider sz = new Slider(-1000, 1000, z);
	    final Label lx = new Label(Double.toString(sx.getValue()));
	    final Label ly = new Label(Double.toString(sy.getValue()));
	    final Label lz = new Label(Double.toString(sz.getValue()));
	    final Label nameX = new Label("X");
	    final Label nameY = new Label("Y");
	    final Label nameZ = new Label("Z");


	    sx.setShowTickLabels(true);
	    sy.setShowTickLabels(true);
	    sz.setShowTickLabels(true);
	    sx.setMajorTickUnit(500);
	    sy.setMajorTickUnit(500);
	    sz.setMajorTickUnit(500);
	    sx.setMinorTickCount(3);
	    sy.setMinorTickCount(3);
	    sz.setMinorTickCount(3);

	    nameX.setLayoutX(50);
	    nameX.setLayoutY(50);
	    nameY.setLayoutX(50);
	    nameY.setLayoutY(150);
	    nameZ.setLayoutX(50);
	    nameZ.setLayoutY(250);

	    sx.setLayoutX(100);
	    sx.setLayoutY(50);
	    sy.setLayoutX(100);
	    sy.setLayoutY(150);
	    sz.setLayoutX(100);
	    sz.setLayoutY(250);

	    lx.setLayoutX(250);
	    lx.setLayoutY(50);
	    ly.setLayoutX(250);
	    ly.setLayoutY(150);
	    lz.setLayoutX(250);
	    lz.setLayoutY(250);

	    sx.valueProperty().addListener((
	    		ObservableValue<? extends Number> ov,
	    		Number old_val, Number new_val) -> {
	    			x = (double) new_val;
	    			camera.setTranslateX(x);
	    			lx.setText(String.format("%.2f", new_val));
	    		}
	    		);

	    sy.valueProperty().addListener((
	    		ObservableValue<? extends Number> ov,
	    		Number old_val, Number new_val) -> {
	    			y = (double) new_val;
	    			camera.setTranslateY(y);
	    			ly.setText(String.format("%.2f", new_val));
	    		}
	    		);

	    sz.valueProperty().addListener((
	    		ObservableValue<? extends Number> ov,
	    		Number old_val, Number new_val) -> {
	    			z = (double) new_val;
	    			camera.setTranslateZ(z);
	    			lz.setText(String.format("%.2f", new_val));
	    		}
	    		);


	    Group root2 = new Group();
	    root2.getChildren().addAll(sx, sy, sz, lx, ly, lz, nameX, nameY, nameZ);
	    Scene scene = new Scene(root2, 400, 1000);

	    this.setScene(scene);
	}

}
