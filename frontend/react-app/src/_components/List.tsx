import React, { Dispatch, SetStateAction } from "react";
import { YoutubeList } from "../_utils/type";
import { List, ListItem, ListItemButton, ListItemText } from "@mui/material";

export const VideoList = ({ youtube_list, setIndex }: { youtube_list: YoutubeList | null, setIndex: Dispatch<SetStateAction<number | null>> }) => {
    if (!youtube_list) return <List></List>;
    return (

        <List
            sx={{
                width: '100%',
                position: 'relative',
                height: "100%",
                overflow: "auto",
                // '&::-webkit-scrollbar': {
                //     display: "none"
                // },
                "&::- webkit - scrollbar": {
                    width: "7px"
                },
            }}>
            {
                youtube_list.map((snippet, index) => (
                    <ListItemButton onClick={() => setIndex(index)}>
                        <ListItem key={index}>
                            <ListItemText
                                primary={snippet.title} secondary={snippet.channelTitle} />
                        </ListItem>
                    </ListItemButton>
                ))
            }
        </List >

    )
}