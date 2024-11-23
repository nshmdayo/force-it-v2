package application3;

import java.util.ArrayList;

import KinectPV2.KJoint;
import javafx.animation.AnimationTimer;
import javafx.application.Application;
import javafx.concurrent.ScheduledService;
import javafx.concurrent.Task;
import javafx.event.EventHandler;
import javafx.geometry.Point3D;
import javafx.scene.Group;
import javafx.scene.PerspectiveCamera;
import javafx.scene.PointLight;
import javafx.scene.Scene;
import javafx.scene.SceneAntialiasing;
import javafx.scene.input.KeyEvent;
import javafx.scene.paint.Color;
import javafx.scene.transform.Affine;
import javafx.stage.Stage;
import javafx.stage.StageStyle;
import processing.core.PApplet;

public class Main extends Application {

	public static final int SCENE_WIDTH = 1920;
	public static final int SCENE_HEIGHT = 1080;

	public static final int SPHERE_WIDTH_NUMBER = 21;
	public static final int SPHERE_HEIGHT_NUMBER = 21;

	public static final double CAMERA_X = 0;
	public static final double CAMERA_Y = -100;
	public static final double CAMERA_Z = -250;

	private Group root = new Group();
	private MyTimer loop;

	private Vec rightVec = new Vec();

	private ArrayList<Perticle> wallBalls = new ArrayList<Perticle>();
	private ArrayList<Perticle> forces = new ArrayList<Perticle>();

	long t;
	private final double m1 = 1, m2 = 1; // m1 = wallPerticles, m2 = forces
	private final double a = 1 / (m1 + m2);

	KJoint[] joints;
	private Vec right = new Vec();
	private Vec left = new Vec();
	private Vec spineS = new Vec();
	private Vec base = new Vec();
	private Vec shoulderR = new Vec();

	int makeForceNum = 500;

	double FORCE_POWER_RATE = 0.4;

	private final double ballradius = 5, forceradius = 1;
	private final double s = Math.pow(ballradius + forceradius, 2);

	private Affine rotate, translate, scale;

	public static int d = 10;

	public void start(Stage stage){
		try {
			Group wall = new Group();
			for(int y = -20; y < 0; y++) {
				for(int x = -20; x < 20; x++) {
					Perticle ball = new Perticle(x*ballradius*2, y*ballradius*2, 0);
					ball.setRadius(ballradius);
					wallBalls.add(ball);
				}
			}
			wall.getChildren().addAll(wallBalls);
			root.getChildren().add(wall);

			PointLight light = new PointLight();
//			light.setColor(Color.RED);
			light.setTranslateX(1000);
			light.setTranslateZ(-1000);
			root.getChildren().add(light);

			PerspectiveCamera camera = new PerspectiveCamera(true);
			camera.setTranslateY(CAMERA_Y);
			camera.setTranslateZ(CAMERA_Z);
			camera.setFieldOfView(45);
			camera.setFarClip(1000);

			Scene s = new Scene(root, SCENE_WIDTH, SCENE_HEIGHT, true, SceneAntialiasing.BALANCED);
			s.setFill(Color.BLACK);
			s.setCamera(camera);

			stage.initStyle(StageStyle.UNDECORATED);
			stage.setScene(s);
			stage.show();

			loop = new MyTimer();
			loop.setRestartOnFailure(true);

			Stage paramStage = new ParamWindow(camera);
		    paramStage.setX(50);
		    paramStage.show();

		    EventHandler<KeyEvent> calibration = (event) -> {
	        	System.out.println("Ready....");

	        	joints = KinectDevice.skeleton.getJoints();
	        	KJoint b = joints[KinectDevice.kinect.JointType_SpineBase];
	        	KJoint m = joints[KinectDevice.kinect.JointType_SpineMid];
	        	double tmp = b.getZ()/2.0;
	        	if(tmp > 1.0) tmp = 1.0;
	        	double theta = Math.acos(tmp);
	        	System.out.println(Math.toDegrees(theta));

	        	rotate = new Affine(1, 0, 0, 0, 0, Math.cos(theta), -Math.sin(theta), 0, 0, Math.sin(theta), Math.cos(theta), 0);
	        	Point3D m2 = new Point3D(m.getX(), m.getY(), m.getZ());
	        	m2 = rotate.transform(m2);
	        	translate = new Affine(1, 0, 0, 0, 0, 1, 0, -m.getY(), 0, 0, 1, 0);
	        	scale = new Affine(100, 0, 0, 0, 0, -100, 0, 0, 0, 0, -100, 0);

				t = System.currentTimeMillis();
	        	loop.restart();

	        	System.out.println("Start!!");
		    };
		    stage.addEventFilter(KeyEvent.KEY_TYPED, calibration);

			new AnimationTimer() {

				public void handle(long now) {

					for(int y = d; y < 20-d; y++) {
						for(int x = d; x < 40-d; x++) {
							for(int j = -d; j <= d; j++) {
								for(int i = -d; i <= d; i++) {
									int a = (y+j) * 40 + x+i;
									Vec temp = new Vec(wallBalls.get(a).getTranslateX(), wallBalls.get(a).getTranslateY(), wallBalls.get(a).getTranslateZ());
									wallBalls.get(a).setAroundMojule(temp);
								}
							}
						}
					}

					for(Perticle wb : wallBalls) {
						wb.gravity();
						wb.moduleGravity();
						wb.move();
					}
				}
			}.start();
		} catch (Exception e) {
			e.printStackTrace();
		}
	}

	class MyTimer extends ScheduledService<Void> {

		@Override
		protected Task<Void> createTask() {
			return new MyTask();
		}
	}

	class MyTask extends Task<Void> {

		@Override
		protected Void call() throws Exception {
			if(KinectDevice.skeletonList.size() > 0) {
				recognize();
				makeForce();
			}
			collision();
			move();
			return null;
		}

		public void recognize() {
			joints = KinectDevice.skeleton.getJoints();
			KJoint r = joints[KinectDevice.kinect.JointType_HandRight];
			KJoint l = joints[KinectDevice.kinect.JointType_HandLeft];
			KJoint ss = joints[KinectDevice.kinect.JointType_SpineShoulder];
			KJoint b = joints[KinectDevice.kinect.JointType_SpineBase];
			KJoint sr = joints[KinectDevice.kinect.JointType_ShoulderRight];

			right = new Vec(r.getX(), r.getY(), r.getZ());
			left = new Vec(l.getX(), l.getY(), l.getZ());
			spineS = new Vec(ss.getX(), ss.getY(), ss.getZ());
			base = new Vec(b.getX(), b.getY(), b.getZ());
			shoulderR = new Vec(sr.getX(), sr.getY(), sr.getZ());

			rightVec.reset();

			int a = r.getState();
			if(a == KinectDevice.kinect.HandState_Open) {
				rightVec.sub(right, shoulderR);
			}
		}

		public void makeForce() {
			Point3D rightHand = new Point3D(right.x, right.y, right.z); // どうなんやろ，この初期化でいいのか
			rightHand = rotate.transform(rightHand);
			rightHand = translate.transform(rightHand);
			rightHand = scale.transform(rightHand);

			Point3D leftHand = new Point3D(left.x, left.y, left.z);
			leftHand = rotate.transform(leftHand);
			leftHand = translate.transform(leftHand);
			leftHand = scale.transform(leftHand);

			Point3D spineShoulder = new Point3D(spineS.x, spineS.y, spineS.z);
			spineShoulder = rotate.transform(spineShoulder);
			spineShoulder = translate.transform(spineShoulder);
			spineShoulder = scale.transform(spineShoulder);

			Point3D spineBase = new Point3D(base.x, base.y, base.z);
			spineBase = rotate.transform(spineBase);
			spineBase = translate.transform(spineBase);
			spineBase = scale.transform(spineBase);

			rightVec.mult(FORCE_POWER_RATE);
			Point3D rightV = new Point3D(rightVec.x, rightVec.y, rightVec.z);
			rightV = rotate.transform(rightV);
			rightV = translate.transform(rightV);
			rightV = scale.transform(rightV);

			double d = Math.pow(spineBase.getX()-leftHand.getX(), 2) + Math.pow(spineBase.getY()-leftHand.getY(), 2) + Math.pow(spineBase.getZ()-leftHand.getZ(), 2);
			d = Math.sqrt(d);

			if(rightV.getX() != 0 && rightV.getY() != 0 && rightV.getZ() != 0) {
				for(int i = 0; i < makeForceNum; i++) {
					double d2x = d * (Math.random()-0.5) * 0.05;
					double d2y = d * (Math.random()-0.5) * 0.05;
					double d2z = d * (Math.random()-0.5) * 0.05;
					Perticle f = new Perticle(rightHand.getX()*d2x, rightHand.getY()*d2y, rightHand.getZ()*d2z);
					f.setRadius(forceradius);
					Vec temp = new Vec(rightV.getX(), rightV.getY(), rightV.getZ());
					f.addVelocity(temp);
					forces.add(f);
				}
			}
		}

		public void collision() {
			for(int i = 0; i < wallBalls.size(); i++) {
				for(int j = 0; j < forces.size(); j++) {
					if(i == j)
						continue;
					if(collisionFlag(wallBalls.get(i), forces.get(j))) {
						Vec v1 = new Vec(); // v1 = wallPerticles
						Vec v2 = new Vec(); // v2 = temp
						v1.copy(wallBalls.get(i).getVelocity());
						v1.mult(m1 - m2);
						v2.copy(forces.get(j).getVelocity());
						v2.mult(2 * m2);
						v1.add(v2);
						v1.mult(a);
						wallBalls.get(i).addVelocity(v1);
						forces.get(i).delateFlag = true;
					}
				}
			}

			for(int i = 0; i < forces.size(); i++) {
				if(forces.get(i).delateFlag)
					forces.remove(i);
			}
		}

		public void move() {
			for(Perticle p : forces) {
				p.move();
			}
			for(int i = 0; i < forces.size(); i++) {
				if(System.currentTimeMillis() - forces.get(i).t > 5000)
					forces.remove(i);
			}
		}

		public boolean collisionFlag(Perticle ball, Perticle force) {
			double dx = Math.pow(ball.getTranslateX()-force.getTranslateX(), 2);
			double dy = Math.pow(ball.getTranslateY()-force.getTranslateY(), 2);
			double dz = Math.pow(ball.getTranslateZ()-force.getTranslateZ(), 2);
			double d = dx + dy + dz;
			if(d < s)
				return true;
			return false;
		}

	}

	public static void main(String[] args) {
		PApplet.main(new String[]{KinectDevice.class.getName()});
		launch(args);
	}
}
