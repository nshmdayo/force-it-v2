package application3;

import java.util.ArrayList;

import KinectPV2.KSkeleton;
import KinectPV2.KinectPV2;
import processing.core.PApplet;

public class KinectDevice extends PApplet{
	public static KinectPV2 kinect;
	public static ArrayList<KSkeleton> skeletonList;
	public static KSkeleton skeleton;

	public void settings(){
		size(512, 424, P3D);
	}

	public void setup(){
		skeletonList = new ArrayList<>();
		kinect = new KinectPV2(this);
		kinect.enableDepthMaskImg(true);
		kinect.enableSkeletonDepthMap(true);
		kinect.enableSkeleton3DMap(true);
		kinect.init();
	}

	public void draw(){
		image(kinect.getDepthMaskImage(), 0, 0);

		skeletonList = kinect.getSkeleton3d();

		int index = 0;
		if(skeletonList.size() != 0){
			float min = skeletonList.get(0).getJoints()[KinectDevice.kinect.JointType_SpineBase].getZ();
			for(int i = 0; i < skeletonList.size() - 1; i++){
				float now = skeletonList.get(i).getJoints()[KinectDevice.kinect.JointType_SpineBase].getZ();
				if(min > now){
					index = i;
					min = now;
				}
			}
			skeleton = skeletonList.get(index);
		}
	}

//	public static void main(String[] args) {
//		PApplet.main(new String[]{MyKinectDevice.class.getName()});
//	}
}
