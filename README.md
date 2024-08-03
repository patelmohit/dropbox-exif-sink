# dropbox-exif-sink
Move Dropbox files to your local machine, with exif metadata enrichment.

## Background
You might be living in the modern age, taking a lot of pretty photos with your iPhone/Android phones. At some point, you ran into a problem - "I have too many photos, and I am storing them all in the cloud - paying extra money per month. Is there a way to save on those costs?? Am I doomed to never owning anything anymore??"

Have no fear, the dropbox-exif-sink project is here! Using Dropbox as an intermediary (since you can have a couple of terabytes of free storage), you should enable syncing your photos to Dropbox on your mobile device, and enable the Dropbox service on the computer you want to sync photos to. At that point, run this script periodically (maybe a daily cron, or on your system startup), and it will move the Dropbox files to your machine! Fantastic! Really riveting stuff.

But wait - there's more! Since this is my project, I can do what I want. And performing EXIF metadata enrichment on my photos and videos would really help with organizing them later. What is EXIF metadata? A cool thing about taking photos with cameras is that they enrich your photos with a lot of cool metadata, like when the photo was taken, or even the type of camera settings you used (like aperature, ISO, etc). In my case, the CreateDate is generally the most useful metadata, and I will be renaming the files according to their timestamp, so I can organize my photos and videos by year.

Hopefully you find that useful. Hopefully you like Rust. Hopefully this project has the slight chance of helping someone else. I'm not sure. It's lonely to write a README that no one else will probably read. Oh well, this project will either be a banger or abandoned, we will see.

